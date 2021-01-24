#include "Texture.h"

// Include all implementations
#include "OpenGL/OpenGLTexture.h"

namespace Vertex
{
    Texture2D* Texture2D::Create(const char* path)
    {
        return new OpenGLTexture2D(path);
    }
}
