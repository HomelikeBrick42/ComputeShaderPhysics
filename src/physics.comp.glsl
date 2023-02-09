#version 440 core

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

uniform float u_TS;

struct Circle {
  vec2 position;
  vec2 velocity;
};

layout(std140, binding = 0) readonly buffer InCircleBuffer {
  Circle in_circles[];
};
layout(std140, binding = 1) writeonly buffer OutCircleBuffer {
  Circle out_circles[];
};

void main() {
  int index = int(gl_GlobalInvocationID);
  Circle circle = in_circles[index];

  circle.position += circle.velocity * u_TS;

  out_circles[index] = circle;
}
