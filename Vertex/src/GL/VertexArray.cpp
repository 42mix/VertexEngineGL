#include "VertexArray.h"

// Include all implementations
#include "OpenGL/OpenGLVertexArray.h"

namespace Vertex
{
    VertexArray* VertexArray::Create() { return new OpenGLVertexArray(); }
}