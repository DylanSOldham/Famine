#pragma once

class Mesh {
public:
    Mesh(float* vertices) 
        : vertices(vertices)
    {
    }

private:
    float* vertices;
};