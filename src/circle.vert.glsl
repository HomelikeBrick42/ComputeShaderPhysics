#version 440 core

layout(location = 0) in vec2 a_Position;
layout(location = 1) in vec2 a_UV;

layout(location = 0) out flat int v_InstanceID;
layout(location = 1) out vec2 v_Position;
layout(location = 2) out vec2 v_UV;

uniform ivec2 u_ScreenSize;
uniform vec2 u_CameraPosition;
uniform float u_CameraScale;
uniform vec2 u_WorldOffset;

struct Circle {
  vec2 position;
  vec2 velocity;
  int typ;
};

layout(std430, binding = 0) readonly buffer CircleBuffer { Circle circles[]; };

void main() {
  float aspect = float(u_ScreenSize.x) / float(u_ScreenSize.y);
  vec2 position = a_Position + circles[gl_InstanceID].position + u_WorldOffset;
  v_Position = position;
  position -= u_CameraPosition;
  position /= u_CameraScale;
  gl_Position = vec4(position.x / aspect, position.y, 0.0, 1.0);
  v_UV = a_UV;
  v_InstanceID = gl_InstanceID;
}
