#include "GraphicsContext.h"

#include "OpenGL/OpenGLContext.h"

namespace Vertex
{
    GraphicsContext* GraphicsContext::Create(GLFWwindow* window_handle)
    {
        return new OpenGLContext(window_handle);
    }
}