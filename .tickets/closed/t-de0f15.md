---
id: t-de0f15
title: Fix tkr close command - reports success but doesn't update ticket status or move files from in_progress to closed directory
status: closed
deps: []
links: []
created: 2026-03-22T04:50:50.223653Z
type: task
priority: 2
notes:
- timestamp: 2026-03-22T05:00:21.112564Z
  content: Fixed the update_status method to properly move ticket files between status directories. The issue was that save_ticket only created new files in the target directory but didn't remove the old file from the source directory. Updated the method to remove the old file after saving to the new location. Also fixed test expectations to look for files in their correct status directories.
- timestamp: 2026-03-22T06:35:11.744317Z
  content: 'Added comprehensive regression tests to prevent this bug from recurring: 1) test_ticket_file_movement_between_status_directories - verifies files are properly moved between status directories and old files are removed, 2) test_no_duplicate_files_created_during_status_changes - ensures exactly one file exists after each status change. These tests will catch any future regressions in the file movement logic.'
---

# Fix tkr close command - reports success but doesn't update ticket status or move files from in_progress to closed directory


## Notes

**2026-03-22 05:00:21**: Fixed the update_status method to properly move ticket files between status directories. The issue was that save_ticket only created new files in the target directory but didn't remove the old file from the source directory. Updated the method to remove the old file after saving to the new location. Also fixed test expectations to look for files in their correct status directories.
**2026-03-22 06:35:11**: Added comprehensive regression tests to prevent this bug from recurring: 1) test_ticket_file_movement_between_status_directories - verifies files are properly moved between status directories and old files are removed, 2) test_no_duplicate_files_created_during_status_changes - ensures exactly one file exists after each status change. These tests will catch any future regressions in the file movement logic.