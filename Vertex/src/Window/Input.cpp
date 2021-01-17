#include "Input.h"

// Include all implementations of Input
#include "Window/GLFW/GLFWInput.h"

namespace Vertex
{
    Input* Input::s_Instance = new GLFWInput();
}