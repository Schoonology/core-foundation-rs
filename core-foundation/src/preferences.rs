// Copyright 2020 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Core Foundation Preferences API

use base::TCFType;
pub use core_foundation_sys::preferences::*;
use string::CFString;

pub fn get_boolean(key: CFString, appId: CFString) -> Option<bool> {
    unsafe {
        let mut key_exists = 0;
        let result = CFPreferencesGetAppBooleanValue(
            key.as_concrete_TypeRef(),
            appId.as_concrete_TypeRef(),
            &mut key_exists,
        );

        if key_exists != 0 {
            Some(result != 0)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dnd_preference() {
        use string::CFString;

        let key = CFString::from_static_string("doNotDisturb");
        let appId = CFString::from_static_string("com.apple.notificationcenterui");

        assert!(get_boolean(key, appId).is_some());
    }
}
