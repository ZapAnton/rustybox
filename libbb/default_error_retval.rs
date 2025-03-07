/*
 * Copyright (C) 2003  Manuel Novoa III  <mjn3@codepoet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* Seems silly to copyright a global variable.  ;-)  Oh well.
 *
 * At least one applet (cmp) returns a value different from the typical
 * EXIT_FAILURE values (1) when an error occurs.  So, make it configurable
 * by the applet.  I suppose we could use a wrapper function to set it, but
 * that too seems silly.
 */
#[no_mangle]
pub static mut xfunc_error_retval: u8 = 1i32 as u8;
