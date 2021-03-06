#include "OpenGLBuffer.h"

namespace Vertex
{
    // -----------------------------------
    // ---------- Vertex Buffer ----------
    // -----------------------------------

    OpenGLVertexBuffer::OpenGLVertexBuffer(float* vertices, size_t size, const BufferLayout& layout) : m_Layout(layout)
    {
        glCreateBuffers(1, &m_ID);
        Bind();
        glBufferData(GL_ARRAY_BUFFER, size, vertices, GL_STATIC_DRAW);
    }

    OpenGLVertexBuffer::~OpenGLVertexBuffer() { glDeleteBuffers(1, &m_ID); }

    void OpenGLVertexBuffer::Bind() const { glBindBuffer(GL_ARRAY_BUFFER, m_ID); }

    void OpenGLVertexBuffer::Unbind() const { glBindBuffer(GL_ARRAY_BUFFER, 0); }

    // ----------------------------------
    // ---------- Index Buffer ----------
    // ----------------------------------

    OpenGLIndexBuffer::OpenGLIndexBuffer(uint32_t* indices, size_t size) : m_Count((size) / (sizeof(uint32_t)))
    {
        glCreateBuffers(1, &m_ID);
        Bind();
        glBufferData(GL_ELEMENT_ARRAY_BUFFER, size, indices, GL_STATIC_DRAW);
    }

    OpenGLIndexBuffer::~OpenGLIndexBuffer() { glDeleteBuffers(1, &m_ID); }

    void OpenGLIndexBuffer::Bind() const { glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, m_ID); }

    void OpenGLIndexBuffer::Unbind() const { glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0); }

    // ----------------------------------
    // --------- Uniform Buffer ---------
    // ----------------------------------

    OpenGLUniformBuffer::OpenGLUniformBuffer(const BufferLayout layout, uint32_t bind_point)
        : m_BindPoint(bind_point), m_Layout(layout)
    {
        glCreateBuffers(1, &m_ID);
        Bind();
        glBufferData(GL_UNIFORM_BUFFER, 2 * layout.GetStride(), NULL, GL_STATIC_DRAW);
        Unbind();
        glBindBufferBase(GL_UNIFORM_BUFFER, bind_point, m_ID);
    }

    OpenGLUniformBuffer::~OpenGLUniformBuffer() { glDeleteBuffers(1, &m_ID); }

    void OpenGLUniformBuffer::Bind() const { glBindBuffer(GL_UNIFORM_BUFFER, m_ID); }

    void OpenGLUniformBuffer::Unbind() const { glBindBuffer(GL_UNIFORM_BUFFER, 0); }
}