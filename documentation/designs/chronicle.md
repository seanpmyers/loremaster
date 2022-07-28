# Chronicle

## Concept

The chronicle interface should be the nexus of the application.  
The idea for chronicles is to provide the user with an organized interface to deal with their life, starting with today.  
For the most part the chronicle should be focused on the current day and the user's activities for that day.  
But rather than isolate each day's information, the goal is to give more meaning to an individuals daily actions in relation to their bigger picture.  
This means that the chronicles functionality works best in tandem with the functionailty from other parts of the application, like goals, habits, protocols, etc.  

One potential issue with this idea is that it requires some additional work upfront from the user.  
The user must provide enough information about themselves in terms of habits, goals, planned events, and actions in order for the chronicle to be filled with information.  
Knowing this, it might be useful to have some sort of demo or reference page that users can interact with to get an idea of what their chronicle page could look like once they provide their information.

## User interface (UI)

### Date

The default chronicle date for this interface should be the local date for the user (not the web server).  
The front-end client should sent the user's local date and request that date's chronicle from the backend web server.
A date picker could be used to navigate to historical chronicles.

### Actions

The chronicle interface should contain a section dedicated to user actions.
Users should be able to record actions they take during the day.
The chronicle should also recommend actions to the user, based on the user's goals, habits, and recent action history.
The interface should also show planned actions the user has already input into the application.

### Documents

d
