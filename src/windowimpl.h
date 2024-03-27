#include <iostream>

#include <glew.h>
#include <GLFW/glfw3.h>
#include <glm.hpp>
#include <gtc/matrix_transform.hpp>

class WindowImpl
{
private:
    GLFWwindow* window;
    int width, height;

    static const char* vertex_shader_text;
    static const char* fragment_shader_text;

    GLuint vertex_buffer;
    GLuint vertex_array;
    GLuint vertex_shader;
    GLuint fragment_shader;
    GLuint program;
    GLint color_location;
    GLint vpos_location;


    static void error_callback(int error, const char* description)
    {
        std::cout << "GLFW Error: " << description << "\n";
        exit(1);
    }

    static void key_callback(GLFWwindow* window, int key, int scancode, int action, int mods)
    {
        if (key == GLFW_KEY_ESCAPE && action == GLFW_PRESS)
        {
            glfwSetWindowShouldClose(window, true);
        }
    }

    void render()
    {
        glfwSwapBuffers(window);
        glViewport(0, 0, width, height);
        glfwGetFramebufferSize(window, &width, &height);
    }

public:
    WindowImpl()
    {
        glfwSetErrorCallback(error_callback);

        if (!glfwInit())
        {
            const char* errorDescription = nullptr;
            glfwGetError(&errorDescription);
            std::cout << "GLFW problem: " << errorDescription << "\n";
        }

        window = glfwCreateWindow(640, 480, "Famine", NULL, NULL);
        glfwMakeContextCurrent(window);

        GLenum error = glewInit();
        if (error != GLEW_OK)
        {
            std::cout << "GLEW problem: " << glewGetErrorString(error) <<"\n";
        }
        
        glfwSwapInterval(1);
 
        glGenBuffers(1, &vertex_buffer);
        glBindBuffer(GL_ARRAY_BUFFER, vertex_buffer);
     
        vertex_shader = glCreateShader(GL_VERTEX_SHADER);
        glShaderSource(vertex_shader, 1, &vertex_shader_text, NULL);
        glCompileShader(vertex_shader);
        
        GLint status;
        glGetShaderiv(vertex_shader, GL_COMPILE_STATUS, &status);
        if (status != GL_TRUE)
        {
            std::cout << "Vertex shader failed to compile" << std::endl;
        }
     
        fragment_shader = glCreateShader(GL_FRAGMENT_SHADER);
        glShaderSource(fragment_shader, 1, &fragment_shader_text, NULL);
        glCompileShader(fragment_shader);
        glGetShaderiv(fragment_shader, GL_COMPILE_STATUS, &status);
        if (status != GL_TRUE)
        {
            std::cout << "Fragment shader failed to compile" << std::endl;
        }
     
        program = glCreateProgram();
        glAttachShader(program, vertex_shader);
        glAttachShader(program, fragment_shader);
        glLinkProgram(program);
     
        color_location = glGetUniformLocation(program, "uColor");
        vpos_location = glGetAttribLocation(program, "vPos");
     
        glGenVertexArrays(1, &vertex_array);
        glBindVertexArray(vertex_array);
        glEnableVertexAttribArray(vpos_location);
        glVertexAttribPointer(vpos_location, 2, GL_FLOAT, GL_FALSE, 2 * sizeof(float), 0);
    }

    ~WindowImpl()
    {
        glfwDestroyWindow(window);
        glfwTerminate();
    }

    void process()
    {
        glfwPollEvents();
        render();
    }

    bool wantsToClose() { return glfwWindowShouldClose(window); }

    void clear()
    {
        glClear(GL_COLOR_BUFFER_BIT);
    }

    void drawRect(float x, float y, float w, float h, float r, float g, float b)
    {
        const float vertices[12] =
        {
            x, y,
            x, y + h,
            x + w, y,
            x, y + h,
            x + w, y + h,
            x + w, y
        };
 
        glUseProgram(program);
        glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);
        glUniform3f(color_location, r, g, b);
        glBindVertexArray(vertex_array);
        glDrawArrays(GL_TRIANGLES, 0, 6);
    }
};

const char* WindowImpl::vertex_shader_text =
    "#version 330\n"
    "uniform vec3 uColor;\n"
    "in vec2 vPos;\n"
    "out vec3 color;\n"
    "void main()\n"
    "{\n"
    "    gl_Position = vec4(vPos, 0.0, 1.0);\n"
    "    color = uColor;\n"
    "}\n";

const char* WindowImpl::fragment_shader_text =
    "#version 330\n"
    "in vec3 color;\n"
    "out vec4 fragment;\n"
    "void main()\n"
    "{\n"
    "    fragment = vec4(color, 1.0);\n"
    "}\n";