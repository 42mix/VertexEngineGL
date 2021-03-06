#pragma once

#include "Window/Window.h"

#include "Window/Input.h"

#include "GLFWInput.h"

namespace Vertex
{
    class GLFWWindow : public Window
    {
    public:
        GLFWWindow(const WindowProperties properties = WindowProperties());
        ~GLFWWindow();

        void OnEvent(Event& event) override;

        void OnUpdate(TimeDelta delta_time) override;

        unsigned int GetWidth() const override { return m_Data.width; }
        unsigned int GetHeight() const override { return m_Data.height; }

        inline void SetEventCallback(std::function<void(Event&)> func) override { m_Data.event_callback = func; }

        inline void SetVSync(bool conf) override
        {
            // glfwSwapInterval((conf) ? 1 : 0);
            // m_Data.v_sync = conf;
            glfwSwapInterval(0);
            m_Data.v_sync = false;
        }

        inline bool IsVSync() const override { return m_Data.v_sync; }

        inline void* GetNativeWindow() const override { return m_Window; }

        inline bool ShouldClose() const override { return glfwWindowShouldClose(m_Window); }

        inline GraphicsContext& GetGraphicsContext() const override { return *m_Context; }

    private:
        bool OnWindowResizeEvent(WindowResizeEvent& event);

    private:
        GLFWwindow*                      m_Window;
        std::shared_ptr<GraphicsContext> m_Context;
        WindowProperties                 m_Data;

    private:
        void ShutDown();
    };

}