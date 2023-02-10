#version 440 core

layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

uniform int u_CircleCount;
uniform float u_TS;

struct Circle {
  vec2 position;
  vec2 velocity;
  int typ;
};

layout(std430, binding = 0) readonly buffer InCircleBuffer {
  Circle in_circles[];
};
layout(std430, binding = 1) writeonly buffer OutCircleBuffer {
  Circle out_circles[];
};

float forces[3][3] = float[3][3](
    float[3](0.5, 1.0, 0.0), float[3](1.0, 0.5, 0.0), float[3](0.0, 0.0, 0.5));

float computeForce(float r, float a) {
  const float beta = 0.3;
  if (r < beta) {
    return r / beta - 1;
  } else if (beta < r && r < 1) {
    return a * (1 - abs(2 * r - 1 - beta) / (1 - beta));
  } else {
    return 0.0;
  }
}

void main() {
  int index = int(gl_GlobalInvocationID);
  if (index >= u_CircleCount)
    return;

  Circle circle = in_circles[index];

  for (int i = 0; i < u_CircleCount; i++) {
    if (i != index) {
      vec2 aToB = in_circles[i].position - circle.position;
      float dist = length(aToB);
      aToB = normalize(aToB);

      float force = forces[in_circles[i].typ][circle.typ];
      circle.velocity += aToB * computeForce(dist / 15.0, force) * u_TS;
    }
  }
  circle.position += circle.velocity * u_TS;
  circle.velocity -= circle.velocity * 0.5 * u_TS;

  out_circles[index] = circle;
}
