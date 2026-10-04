"""Production dependency restrictions, shared by source and resolved-tree audits."""
import re
import tomllib

FORBIDDEN = re.compile(
    # Shared settings/protocol types remain; implementation families do not.
    r'^(?:agent(?:_(?!settings$).*)?|copilot(?:_.*)?|edit_prediction(?:_(?!types$).*)?|language_models|'
    r'anthropic|open_ai|google_ai|bedrock|ollama|codestral|open_router|deepseek|'
    r'minimax|mistral|x_ai(?:_.*)?|zed_ai|hang_telemetry|crashes|'
    r'audio|call|channel|collab_ui|livekit(?:_.*)?|libwebrtc|webrtc(?:-.*)?|cpal|rodio)$'
)


def forbidden_dependencies(tree):
    # cargo tree --prefix none --format '{p}' lists one package per line.
    return sorted({line.split()[0] for line in tree.splitlines()
                   if line.split() and FORBIDDEN.fullmatch(line.split()[0])})


def audit_manifests(root):
    problems = []
    for crate in ('zed', 'remote_server'):
        path = root / 'crates' / crate / 'Cargo.toml'
        content = tomllib.loads(path.read_text())
        tables = [content] + list(content.get('target', {}).values())
        for table in tables:
            for section in ('dependencies', 'build-dependencies'):
                for name, spec in table.get(section, {}).items():
                    package = spec.get('package', name) if isinstance(spec, dict) else name
                    # Optional features are checked in the resolved production tree.
                    optional = isinstance(spec, dict) and spec.get('optional', False)
                    if FORBIDDEN.fullmatch(package) and not optional:
                        problems.append(f'{path.relative_to(root)}: {section}.{name}')
    if problems:
        raise ValueError('Forbidden production dependencies:\n' + '\n'.join(problems))


if __name__ == '__main__':
    import sys
    rejected = forbidden_dependencies(sys.stdin.read())
    if rejected:
        raise SystemExit('Forbidden production dependencies:\n'+'\n'.join(rejected))
