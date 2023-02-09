#version 440 core

layout(location = 0) out vec4 o_Color;

layout(location = 0) in vec2 v_UV;

void main() {
  if ((v_UV.x * 2.0 - 1.0) * (v_UV.x * 2.0 - 1.0) +
          (v_UV.y * 2.0 - 1.0) * (v_UV.y * 2.0 - 1.0) >
      0.25) {
    discard;
  }
  o_Color = vec4(1.0, 0.0, 0.0, 1.0);
}
