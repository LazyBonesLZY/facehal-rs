use nativewindow::Surface;
use std::ffi::c_void;

unsafe extern "C" {
    fn facehal_preview_render_nv21(
        window: *mut c_void,
        frame: *const u8,
        frame_size: usize,
        width: i32,
        height: i32,
        sensor_orientation: i32,
    ) -> i32;
}

pub struct Preview {
    surface: Option<Surface>,
    failure_logged: bool,
}

impl Preview {
    pub fn new(surface: Option<Surface>) -> Self {
        if surface.is_some() {
            eprintln!("FaceHAL preview: enrollment surface received");
        } else {
            eprintln!("FaceHAL preview: enrollment surface missing");
        }
        Self {
            surface,
            failure_logged: false,
        }
    }

    pub fn render(&mut self, frame: &[u8], width: i32, height: i32, sensor_orientation: i32) {
        let Some(surface) = self.surface.as_ref() else {
            return;
        };
        let window = raw_surface_pointer(surface);
        if window.is_null() {
            self.log_failure(-1);
            return;
        }
        let status = unsafe {
            facehal_preview_render_nv21(
                window,
                frame.as_ptr(),
                frame.len(),
                width,
                height,
                sensor_orientation,
            )
        };
        if status != 0 {
            self.log_failure(status);
        }
    }

    fn log_failure(&mut self, status: i32) {
        if !self.failure_logged {
            self.failure_logged = true;
            eprintln!("FaceHAL preview: render failed status={status}");
        }
    }
}

fn raw_surface_pointer(surface: &Surface) -> *mut c_void {
    if std::mem::size_of::<Surface>() != std::mem::size_of::<*mut c_void>() {
        return std::ptr::null_mut();
    }
    unsafe { *(std::ptr::from_ref(surface).cast::<*mut c_void>()) }
}
