#version 440 core

layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

uniform int u_CircleCount;
uniform float u_TS;
uniform float u_MaxPos;

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

float forces[4][4] =
    float[4][4](float[4](0.5, 1.0, -0.5, 0.0), float[4](1.0, 1.0, 1.0, 0.0),
                float[4](0.0, 0.0, 0.5, 2.0), float[4](0.0, 0.0, 0.0, 1.0));

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

vec2 calculateActualForce(int aType, int bType, vec2 a, vec2 b) {
  vec2 aToB = b - a;

  float dist = length(aToB);
  if (abs(dist) < 0.01)
    return vec2(0.0, 0.0);

  aToB = normalize(aToB);

  float force = forces[aType][bType];
  return aToB * computeForce(dist / 16.0, force) * u_TS;
}

void main() {
  int index = int(gl_GlobalInvocationID);
  if (index >= u_CircleCount)
    return;

  Circle circle = in_circles[index];

  for (int i = 0; i < u_CircleCount; i++) {
    circle.velocity += calculateActualForce(
        circle.typ, in_circles[i].typ, circle.position,
        in_circles[i].position + vec2(-u_MaxPos, -u_MaxPos) * 2.0);
    circle.velocity += calculateActualForce(
        circle.typ, in_circles[i].typ, circle.position,
        in_circles[i].position + vec2(0.0, -u_MaxPos) * 2.0);
    circle.velocity += calculateActualForce(
        circle.typ, in_circles[i].typ, circle.position,
        in_circles[i].position + vec2(u_MaxPos, -u_MaxPos) * 2.0);
    circle.velocity += calculateActualForce(
        circle.typ, in_circles[i].typ, circle.position,
        in_circles[i].position + vec2(-u_MaxPos, 0.0) * 2.0);
    circle.velocity +=
        calculateActualForce(circle.typ, in_circles[i].typ, circle.position,
                             in_circles[i].position + vec2(0.0, 0.0) * 2.0);
    circle.velocity += calculateActualForce(
        circle.typ, in_circles[i].typ, circle.position,
        in_circles[i].position + vec2(u_MaxPos, 0.0) * 2.0);
    circle.velocity += calculateActualForce(
        circle.typ, in_circles[i].typ, circle.position,
        in_circles[i].position + vec2(-u_MaxPos, u_MaxPos) * 2.0);
    circle.velocity += calculateActualForce(
        circle.typ, in_circles[i].typ, circle.position,
        in_circles[i].position + vec2(0.0, u_MaxPos) * 2.0);
    circle.velocity += calculateActualForce(
        circle.typ, in_circles[i].typ, circle.position,
        in_circles[i].position + vec2(u_MaxPos, u_MaxPos) * 2.0);
  }
  circle.position += circle.velocity * u_TS;
  circle.velocity -= circle.velocity * 4.0 * u_TS;

  if (circle.position.x > u_MaxPos)
    circle.position.x -= 2.0 * u_MaxPos;
  if (circle.position.x < -u_MaxPos)
    circle.position.x += 2.0 * u_MaxPos;
  if (circle.position.y > u_MaxPos)
    circle.position.y -= 2.0 * u_MaxPos;
  if (circle.position.y < -u_MaxPos)
    circle.position.y += 2.0 * u_MaxPos;

  out_circles[index] = circle;
}
