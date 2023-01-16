# Chronicled Actions

> Recording of actions that are taken or the intention of taking certain actions through chronicles.

## Requirements

- [ ] database schema
  - [ ] intention
    - [x] action
- [ ] Can record an intended action for today's chronicle
  - [ ] back-end
    - [ ] database
      - [ ] relation to intention
      - [ ] relation to action
      - [ ] relation to person
      - [ ] relation to chronicle
      - [ ] relation to completed action
      - [ ] optional timestamp
      - [ ] create, read, update, delete
    - [ ] handler
      - [ ] add handlers for create, read, update, delete
    - [ ] router
      - [ ] add routes for create, read, update, delete
  - [ ] front-end
    - [ ] create user interface component showing intentions
      - [ ] on chronicle page load, make API call to show list of intentions
      - [ ] list intentions with their meta-data
      - [ ] add button to delete individual intentions
      - [ ] add component to create new intention for the day
        - [ ] create save/add button
        - [ ] create action selection component
        - [ ] reload list component on addition
- [ ] Can record an action taken for today's chronicle
  - [ ] back-end
    - [ ] database
      - [ ] add queries for create, read, update, delete
      - [ ] relation to action
      - [ ] relation to completed action
      - [ ] relation to person
      - [ ] relation to chronicle
      - [ ] optional timestamp
    - [ ] handler
      - [ ] add handlers for create, read, update, delete
    - [ ] router
      - [ ] add routes for create, read, update, delete
  - [ ] front-end
    - [ ] create user interface component showing intentions
      - [ ] on chronicle page load, make API call to show list of intentions
      - [ ] list intentions with their meta-data
      - [ ] add button to delete individual intentions
      - [ ] add component to create new intention for the day
        - [ ] create save/add button
        - [ ] create action selection component
        - [ ] reload list component on addition
