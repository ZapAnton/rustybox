use crate::archival::libarchive::bb_archive::archive_handle_t;
use crate::archival::libarchive::bb_archive::file_header_t;
use crate::archival::libarchive::bb_archive::hardlinks_t;
use crate::libbb::llist::llist_t;
use crate::librb::bb_uidgid_t;
use crate::librb::smallint;
use crate::librb::uoff_t;
use libc;
use libc::off_t;

/*
 * Copyright (C) 2002 by Glenn McGrath
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* Accept any non-null name, its not really a filter at all */
#[no_mangle]
pub unsafe extern "C" fn filter_accept_all(
  mut archive_handle: *mut archive_handle_t,
) -> libc::c_char {
  if !(*(*archive_handle).file_header).name.is_null() {
    return 0i32 as libc::c_char;
  }
  return 1i32 as libc::c_char;
}
