"""Shared fixtures and process isolation for the fork's macOS runtime checks."""
import ctypes
import json
import os
from pathlib import Path
import shutil
import subprocess
import time


def executable(app):
    app = Path(app).resolve()
    if app.is_dir():
        app = app / 'Contents/MacOS/zed'
    if not app.is_file():
        raise ValueError(f'Application binary not found: {app}')
    return app


def profile(root, proxy=None):
    root = Path(root)
    config = root / 'config'
    config.mkdir(parents=True)
    settings = {'disable_ai': True, 'auto_update': False,
                'telemetry': {'metrics': False, 'diagnostics': False},
                'auto_install_extensions': {'html': False},
                'auto_update_extensions': {'html': False}}
    if proxy:
        settings['proxy'] = proxy
    (config / 'settings.json').write_text(json.dumps(settings))
    return root


def notebook(count):
    return {'nbformat': 4, 'nbformat_minor': 5,
            'metadata': {'language_info': {'name': 'python'}},
            'cells': [{'cell_type': 'code', 'id': f'cell-{i}', 'metadata': {},
                       'source': [f'print({i})\n'], 'execution_count': None,
                       'outputs': [{'output_type': 'stream', 'name': 'stdout',
                                    'text': [f'{i}\n']}]} for i in range(count)]}


def launch(binary, data_dir, files, log, proxy=None):
    environment = os.environ.copy()
    if proxy:
        environment.update(HTTP_PROXY=proxy, HTTPS_PROXY=proxy,
                           http_proxy=proxy, https_proxy=proxy,
                           NO_PROXY='', no_proxy='')
    return subprocess.Popen([str(binary), '--user-data-dir', str(data_dir), *map(str, files)],
                            stdout=log, stderr=log, env=environment, start_new_session=True)


def stop(process):
    # Terminate only the process launched by this check, never the user's editor.
    if process.poll() is None:
        process.terminate()
        try:
            process.wait(timeout=10)
        except subprocess.TimeoutExpired:
            process.kill()
            process.wait(timeout=10)


def retain_logs(data_dir, process_log, destination, name):
    """Keep only this disposable profile's logs for failed CI diagnosis."""
    destination = Path(destination) / name
    destination.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(process_log, destination / 'process.log')
    for log in Path(data_dir).rglob('*.log'):
        target = destination / log.relative_to(data_dir)
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(log, target)


class MacWindows:
    """Observe the first visible layer-zero window owned by our exact process."""
    def __init__(self):
        self.cg = ctypes.CDLL('/System/Library/Frameworks/CoreGraphics.framework/CoreGraphics')
        self.cf = ctypes.CDLL('/System/Library/Frameworks/CoreFoundation.framework/CoreFoundation')
        self.cg.CGWindowListCopyWindowInfo.argtypes = [ctypes.c_uint32, ctypes.c_uint32]
        self.cg.CGWindowListCopyWindowInfo.restype = ctypes.c_void_p
        self.cf.CFArrayGetCount.argtypes = [ctypes.c_void_p]
        self.cf.CFArrayGetCount.restype = ctypes.c_long
        self.cf.CFArrayGetValueAtIndex.argtypes = [ctypes.c_void_p, ctypes.c_long]
        self.cf.CFArrayGetValueAtIndex.restype = ctypes.c_void_p
        self.cf.CFDictionaryGetValue.argtypes = [ctypes.c_void_p, ctypes.c_void_p]
        self.cf.CFDictionaryGetValue.restype = ctypes.c_void_p
        self.cf.CFNumberGetValue.argtypes = [ctypes.c_void_p, ctypes.c_int, ctypes.c_void_p]
        self.cf.CFNumberGetValue.restype = ctypes.c_bool
        self.cf.CFRelease.argtypes = [ctypes.c_void_p]
        self.owner = ctypes.c_void_p.in_dll(self.cg, 'kCGWindowOwnerPID').value
        self.layer = ctypes.c_void_p.in_dll(self.cg, 'kCGWindowLayer').value

    def number(self, dictionary, key):
        number = self.cf.CFDictionaryGetValue(dictionary, key)
        value = ctypes.c_int64()
        if number and self.cf.CFNumberGetValue(number, 4, ctypes.byref(value)):
            return value.value
        return None

    def visible(self, pid):
        windows = self.cg.CGWindowListCopyWindowInfo(17, 0)  # On-screen, exclude desktop.
        if not windows:
            return False
        try:
            return any(self.number(window, self.owner) == pid and self.number(window, self.layer) == 0
                       for window in (self.cf.CFArrayGetValueAtIndex(windows, i)
                                      for i in range(self.cf.CFArrayGetCount(windows))))
        finally:
            self.cf.CFRelease(windows)


def wait_window(process, windows, timeout=45):
    started = time.monotonic()
    while time.monotonic() - started < timeout:
        if process.poll() is not None:
            raise RuntimeError(f'Editor exited before opening a window ({process.returncode})')
        if windows.visible(process.pid):
            return
        time.sleep(0.05)
    raise TimeoutError('No visible editor window; check graphical session and application log')
