
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

int main(int, char const**)
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
    
    Stage stage(resourcePath() + "sansation.ttf", resourcePath() + "exampleStage.json", resourcePath() + "exampleUI.json");
    
    sf::Clock frameClock;
    
    
//    stage.addChannelGroup(ChannelGroup(&stage, "Shutter", ChannelGroupType::Single, {1}));
//    stage.addChannelGroup(ChannelGroup(&stage, "Gobo", ChannelGroupType::Single, {2}));
//    stage.addChannelGroup(ChannelGroup(&stage, "Pan + Tilt", ChannelGroupType::XY, {3, 4}));
//    
//    stage.addFixture(Fixture(&stage, "Scanner", {0, 1, 2}));
    
//    std::vector<int> test = {0, 2};
//    std::shared_ptr<UISwitch> s2  = std::make_shared<UISwitch>(&stage, "On 2", test);
//    s2->setFadeTime(sf::seconds(3));
//    s2->setFadeCurve(FadeCurve::linear);
//    s2->channelValues[0][0] = 100;
//    stage.addUiElement(s2);
//    
//    std::shared_ptr<UIPushButton> p1 = std::make_shared<UIPushButton>(&stage, "On 2", test);
//    p1->setFadeTime(sf::seconds(3));
//    p1->setFadeCurve(FadeCurve::linear);
//    p1->channelValues[0][0] = 100;
//    stage.addUiElement(p1);
//    
//    std::shared_ptr<UIChannel> c1  = std::make_shared<UIChannel>(&stage, 1);
//    c1->setFadeTime(sf::seconds(1));
//    c1->setFadeCurve(FadeCurve::linear);
//    stage.addUiElement(c1);
//    
//    std::shared_ptr<UIChannel> c2  = std::make_shared<UIChannel>(&stage, 1);
//    c2->setFadeTime(sf::seconds(1));
//    c2->setFadeCurve(FadeCurve::linear);
//    stage.addUiElement(c2);

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
