#include "GLFWInput.h"

namespace Vertex
{

    bool GLFWInput::IsKeyPressedImpl(int key)
    {
        GLFWwindow* window = static_cast<GLFWwindow*>(Application::Get().GetWindow().GetNativeWindow());

        auto state = glfwGetKey(window, key);

        return (state == GLFW_PRESS || state == GLFW_REPEAT);
    }

    bool GLFWInput::IsMouseButtonPressedImpl(int button)
    {
        GLFWwindow* window = static_cast<GLFWwindow*>(Application::Get().GetWindow().GetNativeWindow());

        auto state = glfwGetMouseButton(window, button);

        return state == GLFW_PRESS;
    }

    float GLFWInput::GetMouseXImpl()
    {
        GLFWwindow* window = static_cast<GLFWwindow*>(Application::Get().GetWindow().GetNativeWindow());

        double xpos, ypos;

        glfwGetCursorPos(window, &xpos, &ypos);

        return (float)xpos;
    }

    float GLFWInput::GetMouseYImpl()
    {
        GLFWwindow* window = static_cast<GLFWwindow*>(Application::Get().GetWindow().GetNativeWindow());

        double xpos, ypos;

        glfwGetCursorPos(window, &xpos, &ypos);

        return (float)ypos;
    }

}