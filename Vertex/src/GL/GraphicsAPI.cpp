#include "GraphicsAPI.h"

// Include all implementations
#include "OpenGL/OpenGLGraphicsAPI.h"

namespace Vertex
{
    GraphicsAPI* GraphicsAPI::Create()
    {
        return new OpenGLGraphicsAPI();
    }
}