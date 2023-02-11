#version 440 core

layout(location = 0) out vec4 o_Color;

layout(location = 0) in flat int v_InstanceID;
layout(location = 1) in vec2 v_Position;
layout(location = 2) in vec2 v_UV;

uniform float u_MaxPos;

struct Circle {
  vec2 position;
  vec2 velocity;
  int typ;
};

layout(std430, binding = 0) readonly buffer CircleBuffer { Circle circles[]; };

vec3 circle_colors[4] = vec3[4](vec3(1.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                vec3(0.0, 0.0, 1.0), vec3(1.0, 1.0, 0.0));

void main() {
  vec2 uv = v_UV * 2.0 - 1.0;
  if (dot(uv, uv) > 0.5 * 0.5) {
    discard;
  }
  Circle circle = circles[v_InstanceID];
  if (abs(v_Position.x) > u_MaxPos || abs(v_Position.y) > u_MaxPos) {
    o_Color = vec4(1.0);
  } else {
    o_Color = vec4(circle_colors[circle.typ], 1.0);
  }
}
