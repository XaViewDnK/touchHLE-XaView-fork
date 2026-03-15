/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `GKSession` and related peer-to-peer classes.

use crate::objc::{objc_classes, ClassExports};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation GKSession: NSObject

- (id)initWithSessionID:(id)sessionID // NSString*
           displayName:(id)displayName // NSString*
           sessionMode:(i32)sessionMode {
    this
}

- (bool)isAvailable {
    false
}

- (())setAvailable:(bool)available {
}

- (id)delegate {
    nil
}

- (())setDelegate:(id)delegate {
}

@end

@implementation GKPeerPickerController: NSObject
// TODO
@end

@implementation GKSessionDelegate: NSObject
// TODO
@end

};
