#!/usr/bin/env python3
"""Deterministic untrusted-worker fixture; never used by the application."""
import json
import os
from pathlib import Path
import sys
import time

assert sys.argv[1:] == ['serve', '--stdio', '--restricted'], sys.argv
roots = []
workspace = None
version = 0
case = 'normal'
session = str(os.getpid())
for line in sys.stdin:
    request = json.loads(line)
    method = request['method']
    params = request.get('params', {})
    if method == 'cancel':
        continue
    if method == 'initialize':
        assert params['restricted'] is True
        roots = params['roots']
        workspace = params['workspace_id']
        case = (Path(roots[0]['path']) / '.case').read_text()
    with (Path(roots[0]['path']) / '.trace').open('a') as trace:
        trace.write(json.dumps({'pid': os.getpid(), 'method': method, 'params': params}) + '\n')
    if method == 'initialize':
        result = dict(protocol_version=1, result_schema_version=2, workspace_id=workspace,
                      session_id=session, restricted=True, roots=roots,
                      search_modes=['text', 'symbol', 'ranked'],
                      capabilities=dict(network=case == 'network', telemetry=False, hybrid=False,
                                        models=False, cancellation=True, document_overrides=True,
                                        document_versions=True, file_notifications=True),
                      limits=dict(request_bytes=4194304, max_file_bytes=2097152))
    elif method.startswith('index/'):
        version += 1
        result = dict(index_version=version)
    elif method == 'document/update':
        result = dict(document_version=params['version'])
    elif method == 'document/close':
        result = {}
    elif method == 'search':
        assert params['mode'] in ['text', 'symbol', 'ranked']
        assert '!*.ipynb' in params['glob']
        if case == 'hang':
            time.sleep(300)
        if case == 'crash':
            sys.exit(19)
        if case == 'oversized':
            print('x' * (4194304 + 1), flush=True)
            continue
        path = {'escape': '../outside.py', 'notebook': 'notebook.ipynb',
                'symlink': 'linked.py'}.get(case, 'evidence.py')
        result = dict(schema_version=2, workspace_id=workspace,
                      session_id='wrong' if case == 'stale' else session,
                      root_id=params['root_id'], index_version=version,
                      mode=params['mode'], query=params['query'],
                      matched_units=1, returned_units=1, truncated=False, incomplete=False,
                      warnings=[], skipped_files={'total': 0}, results=[dict(
                          workspace_id=workspace, root_id=params['root_id'], index_version=version,
                          path=path, start_line=1, end_line=1, symbol='needle', kind='function',
                          score=1, content='def needle(): pass', excerpt_truncated=False,
                          source='disk', document_version=None, content_hash='1' * 64,
                          source_range=dict(start=dict(line=1, byte_column=0),
                                            end=dict(line=1, byte_column=18), end_exclusive=True))])
    else:
        raise AssertionError(method)
    print(json.dumps(dict(protocol_version=1, id=request['id'], result=result)), flush=True)
