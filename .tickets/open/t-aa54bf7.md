---
id: t-aa54bf7
title: Add version command to display build info
status: open
deps: []
links: []
created: 2026-02-04T02:30:32.725982721Z
type: feature
priority: 2
notes:
- timestamp: 2026-02-04T02:30:43.966217186Z
  content: |-
    Feature request: Add a 'version' command to tkr

    Currently tkr does not have a --version flag or version command. Users need to know which version/build they're using for debugging and support.

    Expected behavior:
    - 'tkr version' command should display version information
    - 'tkr --version' flag should also work (standard CLI convention)
    - Should show: version number, build date, git commit hash if available

    Current behavior:
    - 'tkr --version' shows error: 'unexpected argument --version found'
    - No version command exists in help output

    Using: /home/micro/.local/bin/tkr (the release binary we just built)

    This is a standard CLI feature that most tools provide for user convenience and debugging.
---

# Add version command to display build info


## Notes

**2026-02-04 02:30:43**: Feature request: Add a 'version' command to tkr

Currently tkr does not have a --version flag or version command. Users need to know which version/build they're using for debugging and support.

Expected behavior:
- 'tkr version' command should display version information
- 'tkr --version' flag should also work (standard CLI convention)
- Should show: version number, build date, git commit hash if available

Current behavior:
- 'tkr --version' shows error: 'unexpected argument --version found'
- No version command exists in help output

Using: /home/micro/.local/bin/tkr (the release binary we just built)

This is a standard CLI feature that most tools provide for user convenience and debugging.