#include <iostream>

#include <SFML/Audio.hpp>
#include <SFML/Graphics.hpp>

#ifdef __APPLE__
#ifdef TARGET_OS_MAC
#include "ResourcePath.hpp"
#endif
#endif

#include "Stage.hpp"

int main(int argc, char const** argv)
{
    std::string resourcePath = "res/";
    
    #ifdef __APPLE__
    #ifdef TARGET_OS_MAC
        resourcePath = getBundleResourcePath();
    #endif
    #endif
    
    // Create the main window
    sf::RenderWindow window(sf::VideoMode(800, 600), "BitDMX");
    window.setFramerateLimit(60);

    // Set the Icon
    sf::Image icon;
    if (!icon.loadFromFile(resourcePath + "icon.png")) {
        return EXIT_FAILURE;
    }

    //TODO: Commented this out until SFML #1171 is merged into upstream which fixes a bug in linux which causes a segfault
    //window.setIcon(icon.getSize().x, icon.getSize().y, icon.getPixelsPtr());

    std::string port = "";
    if (argc > 1) {
        port = argv[1];
    }
    
    
    std::string pathToConfig = resourcePath;
    
    if (argc > 2) {
        pathToConfig = argv[2]; //TODO cut filename if filename
    }
    
    std::cout << pathToConfig + "/stage.json" << std::endl;
    
    Stage stage(port, resourcePath + "sansation.ttf", pathToConfig + "/stage.json", pathToConfig + "/ui.json");
    
    sf::Clock frameClock;


    // Start the game loop
    while (window.isOpen())
    {
        stage.setCurrentTime(frameClock.getElapsedTime());
        
        // Process events
        sf::Event event;
        while (window.pollEvent(event))
        {
            // Close window: exit
            if (event.type == sf::Event::Closed) {
                window.close();
            }
            
            // Resized window
            if (event.type == sf::Event::Resized) {
                window.setView(sf::View(sf::FloatRect(0.f, 0.f, window.getSize().x, window.getSize().y)));
            }
            
            if (event.type == sf::Event::MouseButtonPressed) {
                stage.onMousePress(event.mouseButton.x, event.mouseButton.y, event.mouseButton.button);
            }
            
            if (event.type == sf::Event::MouseButtonReleased) {
                stage.onMouseRelease(event.mouseButton.x, event.mouseButton.y, event.mouseButton.button);
            }
            
            if (event.type == sf::Event::MouseMoved) {
                stage.onMouseMove(event.mouseMove.x, event.mouseMove.y);
            }

            if (event.type == sf::Event::KeyPressed) {
                stage.onHotkey(event.key.code);
            }
            
            if (event.type == sf::Event::KeyReleased) {
                stage.onHotkeyRelease(event.key.code);
            }
            
            if (event.type == sf::Event::MouseWheelMoved) {
                stage.onScroll(event.mouseWheel.delta);
            }
        }
        
        stage.updateAllChannels();

        // Clear screen
        window.clear(sf::Color (50, 21, 100));

        // Draw the stage
        window.draw(stage);
        
        
        // Update the window
        window.display();
    }

    return EXIT_SUCCESS;
}
