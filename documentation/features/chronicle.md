# Chronicle

## Definition

Here are some common definitions of chronicle:

> a factual written account of important or historical events in the order of their occurrence.
> record (a related series of events) in a factual and detailed way.

In the context of the loremaster application, these definitions should be valid for the most part.  
Chronicles in loremaster represent the user's life broken up by individual days.  

## Concept

The chronicle interface should be the nexus of the application.  
The idea for chronicles is to provide the user with an organized interface to deal with their life, starting with today.  
For the most part the chronicle should be focused on the current day and the user's activities for that day.  
But rather than isolate each day's information, the goal is to give more meaning to an individuals daily actions in relation to their bigger picture.  
This means that the chronicles functionality works best in tandem with the functionality from other parts of the application, like goals, habits, protocols, etc.  

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

Recording actions ideally should require little effort from the user.  
Ideally when the user goes to record an action taken, the application should have predicted (or the user has already planned) the action and the user should be able to simply confirm the action was completed.

If a user wants to record an action that is not planned or easily predicted by the application, the interface should provide a means to record the action with minimal input from the user.  
Minimal input is not fully defined at this moment.  
An arbitrary starting point can be that minimal input from the user means that they can record the action in less than ten clicks of their mouse, or a combination of clicks and typing where the clicks are less than five and the typing is less than fifteen keys.

### Documents

Documents recorded and presented on the chronicle interface have not been decided yet.  
For now, a minimum feature is to allow the user to write free form text and attach it to a given chronicle.  
In the future this will most likely be modified or removed.  

### Objectives

General objectives which the user must define should be displayed as secondary content, outside the main content area.  
The objectives should be displayed to provide additional context to the user's day and the actions they take.  
They most likely should be displayed in an unobtrusive way near related actions, so as to reduce the mental effort required by the user to determine what actions to take each day to achieve their objectives.

### Intentions

Intentions should be displayed as secondary content, outside the main content area.  
Intentions should be shown for not just the current day, but also future events.  
These future intentions should help remind the user to take actions today if possible to prepare for future events and objectives.  
