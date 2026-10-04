import importlib.util
from pathlib import Path
import sys
import unittest

SCRIPT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(SCRIPT))
from no_ai_policy import forbidden_dependencies, audit_manifests

spec = importlib.util.spec_from_file_location('privacy', SCRIPT / 'privacy-no-ai-mac.py')
privacy = importlib.util.module_from_spec(spec)
spec.loader.exec_module(privacy)


class PrivacyChecks(unittest.TestCase):
    def test_recording_proxy_detects_real_requests_and_does_not_forward(self):
        recorder = privacy.Recorder()
        try:
            recorder.positive_control()
            import http.client
            connection = http.client.HTTPConnection('127.0.0.1', recorder.server.server_port, timeout=5)
            connection.request('CONNECT', 'api.anthropic.com:443')
            response = connection.getresponse()
            self.assertEqual(response.status, 502)
            response.read()
            connection.close()
            self.assertEqual(recorder.requests[0]['host'], 'api.anthropic.com')
            self.assertEqual(privacy.violations(recorder.requests), recorder.requests)
        finally:
            recorder.close()

    def test_shared_service_or_unknown_hosts_are_not_silently_allowed(self):
        for host in ['zed.dev', 'cloud.zed.dev', 'new-provider.example', 'github.com']:
            attempts = [{'host': host, 'method': 'CONNECT', 'target': host+':443'}]
            self.assertEqual(privacy.violations(attempts), attempts)
        self.assertEqual(privacy.violations([]), [])

    def test_denylist_catches_families_and_retains_inert_shared_types(self):
        rejected = forbidden_dependencies('agent_new v1\ncopilot_new v1\nwebrtc-sys v1\nchannel v1\nrodio v1\n')
        self.assertEqual(len(rejected), 5)
        self.assertEqual(forbidden_dependencies('telemetry v1\nlanguage_model v1\nagent_settings v1\nedit_prediction_types v1\ntelemetry_events v1\n'), [])

    def test_manifest_check_rejects_new_target_specific_dependency(self):
        import tempfile
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            for name in ['zed', 'remote_server']:
                path = root / 'crates' / name / 'Cargo.toml'
                path.parent.mkdir(parents=True)
                path.write_text('[dependencies]\nserde="1"\n[dev-dependencies]\ncall="1"\n')
            audit_manifests(root)
            with (root / 'crates/zed/Cargo.toml').open('a') as file:
                file.write('[target.\'cfg(target_os = "macos")\'.dependencies]\nstealth={package="agent_new",version="1"}\n')
            with self.assertRaisesRegex(ValueError, 'stealth'):
                audit_manifests(root)


if __name__ == '__main__':
    unittest.main()
