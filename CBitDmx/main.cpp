
//
// Disclaimer:
// ----------
//
// This code will work only if you selected window, graphics and audio.
//
// Note that the "Run Script" build phase will copy the required frameworks
// or dylibs to your application bundle so you can execute it on any OS X
// computer.
//
// Your resource files (images, sounds, fonts, ...) are also copied to your
// application bundle. To get the path to these resources, use the helper
// function `resourcePath()` from ResourcePath.hpp
//

#include <iostream>

#include <SFML/Audio.hpp>
#include <SFML/Graphics.hpp>

// Here is a small helper for you! Have a look.
#include "ResourcePath.hpp"
#include "Stage.hpp"

int main(int argc, char const** argv)
{
    // Create the main window
    sf::RenderWindow window(sf::VideoMode(800, 600), "BitDMX");
    window.setFramerateLimit(60);

    // Set the Icon
    sf::Image icon;
    if (!icon.loadFromFile(resourcePath() + "icon.png")) {
        return EXIT_FAILURE;
    }
    window.setIcon(icon.getSize().x, icon.getSize().y, icon.getPixelsPtr());

    std::string port = "";
    if (argc > 1) {
        port = argv[1];
    }
    
    Stage stage(port, resourcePath() + "sansation.ttf", resourcePath() + "exampleStage.json", resourcePath() + "exampleUI.json");
    
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
