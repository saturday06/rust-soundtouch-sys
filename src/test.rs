use super::*;
use std::str;

#[test]
fn test() {
    unsafe {
        let mut soundtouch = soundtouch_SoundTouch::new();
        println!(
            "soundtouch version: {}",
            soundtouch_SoundTouch_getVersionId()
        );
        soundtouch_SoundTouch_flush(&mut soundtouch);
    }
}
