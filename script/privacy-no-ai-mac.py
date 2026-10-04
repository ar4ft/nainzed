#!/usr/bin/env python3
"""Observe editor startup requests through a recording, non-forwarding HTTP proxy."""
import argparse
import http.client
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import json
import os
from pathlib import Path
import platform
import threading
import tempfile
import time
from urllib.parse import urlsplit

from no_ai_runtime import MacWindows, executable, launch, profile, retain_logs, stop, wait_window


class Recorder:
    def __init__(self):
        self.requests = []
        self.lock = threading.Lock()
        recorder = self

        class Handler(BaseHTTPRequestHandler):
            def capture(self):
                if self.command == 'CONNECT':
                    host = urlsplit('//'+self.path).hostname
                else:
                    host = urlsplit(self.path).hostname
                # Headers and body may contain private data; retain only method/host/path.
                with recorder.lock:
                    recorder.requests.append({'method': self.command, 'host': host, 'target': self.path})
                self.send_error(502, 'Privacy check records attempts without forwarding traffic')

            do_CONNECT = do_GET = do_POST = do_PUT = do_DELETE = do_PATCH = do_OPTIONS = do_HEAD = capture

            def log_message(self, *_):
                pass

        self.server = ThreadingHTTPServer(('127.0.0.1', 0), Handler)
        self.thread = threading.Thread(target=self.server.serve_forever, daemon=True)
        self.thread.start()

    @property
    def url(self):
        return f'http://127.0.0.1:{self.server.server_port}'

    def positive_control(self):
        connection = http.client.HTTPConnection('127.0.0.1', self.server.server_port, timeout=5)
        connection.request('GET', 'http://privacy-control.invalid/check')
        response = connection.getresponse()
        response.read()
        connection.close()
        assert response.status == 502 and self.requests[-1]['host'] == 'privacy-control.invalid'
        self.requests.clear()

    def close(self):
        self.server.shutdown()
        self.server.server_close()
        self.thread.join(timeout=5)


def violations(requests):
    # Fresh plain-text profiles need no network. Shared service domains are not
    # allowlisted: CONNECT hides paths, so permitting them could hide telemetry.
    return list(requests)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--app', required=True)
    parser.add_argument('--output', required=True, type=Path)
    parser.add_argument('--seconds', type=float, default=15)
    args = parser.parse_args()
    if platform.system() != 'Darwin' or args.seconds < 1:
        parser.error('Requires macOS and an observation period of at least 1s')
    recorder = Recorder()
    report = {'scope': 'Fresh plain-text startup through the configured HTTP(S) proxy; no traffic forwarded. Direct sockets and child tools are outside this test.',
              'gpu_emulation_allowed': os.environ.get('ZED_ALLOW_EMULATED_GPU') == '1',
              'platform': platform.platform(), 'observation_seconds': args.seconds}
    try:
        recorder.positive_control()
        report['positive_control_passed'] = True
        with tempfile.TemporaryDirectory(prefix='no-ai-privacy-') as temporary:
            root = Path(temporary)
            data = profile(root / 'profile', recorder.url)
            # Try to re-enable reporting and AI: the fork must still reject both.
            settings_file = data / 'config/settings.json'
            settings = json.loads(settings_file.read_text())
            settings.update(disable_ai=False, telemetry={'metrics': True, 'diagnostics': True})
            settings_file.write_text(json.dumps(settings))
            fixture = root / 'plain.txt'
            fixture.write_text('Privacy startup fixture\n')
            with (root / 'editor.log').open('wb') as log:
                process = launch(executable(args.app), data, [fixture], log, recorder.url)
                try:
                    wait_window(process, MacWindows())
                    started = time.monotonic()
                    while time.monotonic() - started < args.seconds:
                        if process.poll() is not None:
                            raise RuntimeError('Editor exited before completing the privacy observation')
                        time.sleep(0.1)
                    report['editor_window_observed'] = True
                finally:
                    stop(process)
                    log.flush()
                    retain_logs(data, log.name, args.output.parent / 'logs', 'privacy')
    except Exception as error:
        report['error'] = str(error)
    finally:
        recorder.close()
    report['requests'] = recorder.requests
    report['violations'] = violations(recorder.requests)
    report['passed'] = not report.get('error') and not report['violations']
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps(report, indent=2))
    if not report['passed']:
        raise SystemExit(1)


if __name__ == '__main__':
    main()
