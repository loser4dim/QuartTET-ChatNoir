use std::f64::consts;

pub struct Camera {
    eye: [f32;3],
    target: [f32;3],
    up: [f32;3],
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,


    state: CameraState
}

struct CameraState {
    theta: f64,
    phi: f64
}


impl Camera {
    pub fn moving(&mut self, x: f64, y: f64) {
        println!("{}, {}", x, y);
        self.state.phi   = self.state.phi + x / 200.0;
        self.state.theta = self.state.theta + y / 200.0;
        let r = 5.0;

        if self.state.theta < 0.0 {
            self.state.theta = f64::EPSILON;
        }
        if self.state.theta > consts::PI {
            self.state.theta = consts::PI - f64::EPSILON;
        }

        if self.state.phi < 0.0 {
            self.state.phi = f64::EPSILON;
        }
        else if self.state.phi > 2.0 * consts::PI {
            self.state.phi =  2.0 * consts::PI - f64::EPSILON;
        }

        
        self.eye[0] = (r * self.state.theta.sin() * self.state.phi.cos()) as f32;
        self.eye[1] = (r * self.state.theta.sin() * self.state.phi.sin()) as f32;
        self.eye[2] = (r * self.state.theta.cos()) as f32;
    }

    pub fn new() -> Self{
        let theta = consts::PI / 3.0;
        let phi = consts::PI / 6.0;
        let state = CameraState {theta, phi};
        let r = 5.0;

        let target: [f32;3] = [0.0, 0.0, 0.0];
        let eye: [f32;3] = [(r * theta.sin() * phi.cos()) as f32, (r * theta.sin() * phi.sin()) as f32, (r * theta.cos()) as f32];
        let up: [f32;3] = [0.0, 0.0, 1.0];

        let aspect: f32 = 1.0;
        let fovy: f32 = 45.0;
        let znear: f32 = 0.1;
        let zfar: f32 = 100.0;

        return Self{target, eye, up, aspect, fovy, znear, zfar, state};
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.aspect = width / height;
    }

    fn calc_view_matrix(&self) -> [[f32;4];4] {
        let eye    = cgmath::Point3{x: self.eye[0], y: self.eye[1], z: self.eye[2]};
        let target = cgmath::Point3{x: self.target[0], y: self.target[1], z: self.target[2]};
        let up     = cgmath::Vector3{x: self.up[0], y: self.up[1], z: self.up[2]};

        let view_matrix: [[f32;4];4] = cgmath::Matrix4::look_at_rh(eye, target, up).into();
        return view_matrix;
    }

    fn calc_projection_matrix(&self) -> [[f32;4];4] {
        let projection_matrix: [[f32;4];4] = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar).into();
        return projection_matrix;
    }

    pub fn calc_vp_matrix(&self) -> [[f32;4];4] {
        let eye    = cgmath::Point3{x: self.eye[0], y: self.eye[1], z: self.eye[2]};
        let target = cgmath::Point3{x: self.target[0], y: self.target[1], z: self.target[2]};
        let up     = cgmath::Vector3{x: self.up[0], y: self.up[1], z: self.up[2]};

        let view_matrix = cgmath::Matrix4::look_at_rh(eye, target, up);
        let projection_matrix = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        let vp_matrix: [[f32;4];4] = (projection_matrix * view_matrix).into();
        return vp_matrix;
    }
}