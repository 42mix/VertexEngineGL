#include "VertexArray.h"

#include "OpenGL/OpenGLVertexArray.h"

namespace Vertex
{
    VertexArray* VertexArray::Create() { return new OpenGLVertexArray(); }
}