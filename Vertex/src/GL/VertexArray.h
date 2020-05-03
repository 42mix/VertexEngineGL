#pragma once

#include "Buffer.h"

namespace Vertex {

    class VertexArray
    {
    public:
        virtual ~VertexArray() {}

        virtual void Bind() const = 0;
        virtual void Unbind() const = 0;

        virtual void AddVertexBuffer(const std::shared_ptr<VertexBuffer> vbo) = 0;
        virtual void SetIndexBuffer(const std::shared_ptr<IndexBuffer> ibo) = 0;

        static VertexArray* Create();
    };

}

#if defined(VX_RENDER_API_OPENGL)
    #include "OpenGL/OpenGLVertexArray.h"
#endif
// ... per rendering api