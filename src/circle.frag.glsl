#version 440 core

layout(location = 0) out vec4 o_Color;

layout(location = 0) in vec2 v_UV;
layout(location = 1) in flat int v_InstanceID;

struct Circle {
  vec2 position;
  vec2 velocity;
};

layout(std140, binding = 0) readonly buffer CircleBuffer { Circle circles[]; };

void main() {
  if ((v_UV.x * 2.0 - 1.0) * (v_UV.x * 2.0 - 1.0) +
          (v_UV.y * 2.0 - 1.0) * (v_UV.y * 2.0 - 1.0) >
      0.25) {
    discard;
  }
#if 0
  Circle circle = circles[v_InstanceID];
  float sqrSpeed = dot(circle.velocity, circle.velocity);
  o_Color = vec4(
      mix(vec3(0.0, 0.0, 1.0), vec3(1.0, 0.0, 0.0), sqrSpeed / 10000.0), 1.0);
#else
  o_Color = vec4(1.0, 0.0, 0.0, 1.0);
#endif
}
