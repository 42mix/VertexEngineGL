#include "GraphicsAPI.h"

#include "OpenGL/OpenGLGraphicsAPI.h"

namespace Vertex
{
    GraphicsAPI* GraphicsAPI::Create()
    {
        return new OpenGLGraphicsAPI();
    }
}