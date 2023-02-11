#version 440 core

layout(location = 0) out vec4 o_Color;

layout(location = 0) in vec2 v_UV;
layout(location = 1) in flat int v_InstanceID;

struct Circle {
  vec2 position;
  vec2 velocity;
  int typ;
};

layout(std430, binding = 0) readonly buffer CircleBuffer { Circle circles[]; };

vec3 circle_colors[3] =
    vec3[3](vec3(1.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0), vec3(0.0, 0.0, 1.0));

void main() {
  if ((v_UV.x * 2.0 - 1.0) * (v_UV.x * 2.0 - 1.0) +
          (v_UV.y * 2.0 - 1.0) * (v_UV.y * 2.0 - 1.0) >
      0.25) {
    discard;
  }
  Circle circle = circles[v_InstanceID];
  float sqrSpeed = dot(circle.velocity, circle.velocity);
  o_Color = vec4(circle_colors[circle.typ], 1.0);
}
