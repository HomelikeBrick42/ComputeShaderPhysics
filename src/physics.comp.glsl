#version 440 core

layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

uniform int u_CircleCount;
uniform float u_TS;

struct Circle {
  vec2 position;
  vec2 velocity;
};

layout(std430, binding = 0) readonly buffer InCircleBuffer {
  Circle in_circles[];
};
layout(std430, binding = 1) writeonly buffer OutCircleBuffer {
  Circle out_circles[];
};

void main() {
  int index = int(gl_GlobalInvocationID);
  if (index >= u_CircleCount)
    return;

  Circle circle = in_circles[index];

  for (int i = 0; i < u_CircleCount; i++) {
    if (i != index) {
      // TODO: update
    }
  }
  circle.position += circle.velocity * u_TS;
  circle.velocity -= circle.velocity * 0.2 * u_TS;

  out_circles[index] = circle;
}
