#include "Buffer.h"

// Include all implementations
#include "OpenGL/OpenGLBuffer.h"

namespace Vertex
{
    VertexBuffer* VertexBuffer::Create(float* vertices, size_t size, const BufferLayout& layout)
    {
        return new OpenGLVertexBuffer(vertices, size, layout);
    }

    IndexBuffer* IndexBuffer::Create(uint32_t* indices, size_t size)
    {
        return new OpenGLIndexBuffer(indices, size);
    }

    UniformBuffer* UniformBuffer::Create(const BufferLayout layout, uint32_t bind_point)
    {
        return new OpenGLUniformBuffer(layout, bind_point);
    }
}