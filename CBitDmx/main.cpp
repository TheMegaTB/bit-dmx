
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

#include <SFML/Audio.hpp>
#include <SFML/Graphics.hpp>

// Here is a small helper for you! Have a look.
#include "ResourcePath.hpp"
#include "Stage.hpp"
#include "Switch.hpp"

int main(int, char const**)
{
    // Create the main window
    sf::RenderWindow window(sf::VideoMode(800, 600), "SFML window");
    window.setFramerateLimit(60);

    // Set the Icon
    sf::Image icon;
    if (!icon.loadFromFile(resourcePath() + "icon.png")) {
        return EXIT_FAILURE;
    }
    window.setIcon(icon.getSize().x, icon.getSize().y, icon.getPixelsPtr());

//    // Load a sprite to display
//    sf::Texture texture;
//    if (!texture.loadFromFile(resourcePath() + "cute_image.jpg")) {
//        return EXIT_FAILURE;
//    }
//    sf::Sprite sprite(texture);
//
//    // Create a graphical text to display
//    sf::Font font;
//    if (!font.loadFromFile(resourcePath() + "sansation.ttf")) {
//        return EXIT_FAILURE;
//    }
//    sf::Text text("Hello SFML", font, 50);
//    text.setFillColor(sf::Color::Black);
//
//    // Load a music to play
//    sf::Music music;
//    if (!music.openFromFile(resourcePath() + "nice_music.ogg")) {
//        return EXIT_FAILURE;
//    }
//
//    // Play the music
//    music.play();
    
    Stage stage(6, resourcePath() + "sansation.ttf");
    
    sf::Clock frameClock;
    
    stage.addChannelGroup(ChannelGroup(&stage, "Shutter", {1}));
    stage.addChannelGroup(ChannelGroup(&stage, "Gobo", {2}));
    stage.addChannelGroup(ChannelGroup(&stage, "Pan + Tilt", {3, 4, 5}));
    
    stage.addFixture(Fixture(&stage, "Scanner", {0, 1, 2}));
    
    Switch s1 = Switch(&stage, "On 1", {0, 2});
    s1.setFadeTime(sf::seconds(1));
    s1.setFadeCurve(FadeCurve::linear);
    s1.channelValues[0][0] = 50;
    stage.addUiElement(std::make_shared<Switch>(s1));
    
    Switch s2 = Switch(&stage, "On 2", {0, 2});
    s2.setFadeTime(sf::seconds(3));
    s2.setFadeCurve(FadeCurve::linear);
    s2.channelValues[0][0] = 100;
    stage.addUiElement(std::make_shared<Switch>(s2));

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
                stage.onClick(event.mouseButton.x, event.mouseButton.y);
            }

            // Escape pressed: exit
            if (event.type == sf::Event::KeyPressed) {
                stage.onHotkey(event.key.code);
            }
        }
        
        stage.updateAllChannels();

        // Clear screen
        window.clear();

        // Draw the sprite
        window.draw(stage);
        
        
        // Update the window
        window.display();
    }

    return EXIT_SUCCESS;
}
