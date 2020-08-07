#include "Shader.h"

#if defined(VX_RENDER_API_VULKAN)
#else
    #include "OpenGL/OpenGLShader.h"
// ... per rendering API
#endif

namespace Vertex
{
    // helper functions
    const size_t GetSizeOfShaderDataType(ShaderDataType type)
    {
        switch (type)
        {
        case ShaderDataType::Bool:
            return 1;
        case ShaderDataType::Mat3:
            return 36;
        case ShaderDataType::Mat4:
            return 64;
        case ShaderDataType::Int:
            return 4;
        case ShaderDataType::Int2:
            return 8;
        case ShaderDataType::Int3:
            return 12;
        case ShaderDataType::Int4:
            return 16;
        case ShaderDataType::Float:
            return 4;
        case ShaderDataType::Float2:
            return 8;
        case ShaderDataType::Float3:
            return 12;
        case ShaderDataType::Float4:
            return 16;
        }

        CoreLogger::Error("Unknown shader data type, cancelling...");
        return 0;
    };

    const uint32_t GetComponentCountOfShaderDataType(ShaderDataType type)
    {
        switch (type)
        {
        case ShaderDataType::Bool:
            return 1;
        case ShaderDataType::Mat3:
            return 9;
        case ShaderDataType::Mat4:
            return 16;
        case ShaderDataType::Int:
            return 1;
        case ShaderDataType::Int2:
            return 2;
        case ShaderDataType::Int3:
            return 3;
        case ShaderDataType::Int4:
            return 4;
        case ShaderDataType::Float:
            return 1;
        case ShaderDataType::Float2:
            return 2;
        case ShaderDataType::Float3:
            return 3;
        case ShaderDataType::Float4:
            return 4;
        }

        CoreLogger::Error("Unknown shader data type, cancelling...");
        return 0;
    }

    // clang-format off

#if defined(VX_RENDER_API_VULKAN)

    Shader* Shader::Create(const char* vertex_src, const char* fragment_src)
    {
        return nullptr;
    }

#else

    Shader* Shader::Create(const char* vertex_src, const char* fragment_src)
    {
        return new OpenGLShader(vertex_src, fragment_src);
    }

#endif

    // clang-format on
}