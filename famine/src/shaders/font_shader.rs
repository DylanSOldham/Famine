pub const FONT_VERT_SHADER: &str = 
    r##"#version 300 es

    in vec4 v_position;
    in vec2 v_uv;

    out vec2 f_uv;
    out vec3 f_pos;

    void main() {
        gl_Position = v_position;
        f_uv = v_uv;
        f_pos = v_position.xyz;
    }
    "##;
pub const FONT_FRAG_SHADER: &str = 
    r##"#version 300 es
    precision highp float;

    uniform vec4 u_color;
    uniform sampler2D uTexture;

    in vec2 f_uv;
    in vec3 f_pos;

    out vec4 outColor;

    void main() {
        outColor = u_color * texture(uTexture, f_uv).a;
    }
    "##;

