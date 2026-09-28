# Project Architecture

## Project Idea

The goals of the project are as follows:

- Present the user with questions
- asked at regular intervals
- record their responses
- allow the user to view a history of responses
- allow the user to view a graph of numeric responses.

to accomplish these goals we will be writing two applications.

The first is the configuration and review point.
Let's call it "Daily"

This application will handle;
- which questions are available
- which questions are active
- when are questions asked
- and given a single question
    - what is the question
    - what are the allowed kinds of responses
    - when is the question asked
- what questions have been asked
- what were their responses
    - presented in a list or a graph

The second is the interaction point.
Let's call it "Query"

This application will handle;
- what questions need to be asked now
- asking them
- recording their responses

These two applications will communicate via a shared repository or database.

The flow of the application will be twofold

1. Configuration and Review
    - They launch the first application; Daily
    - They coordinate the questions they want to be asked this week
    - They review the Responses they gave last week
    - They close the application

2. Interaction
    - at preset points throughout the day the interaction launches; Query
    - The user sees the question pop-up
    - they can choose to interact with it
    - They answer the question or delay
    - the query is resolved and closes.

## Lowering Ideas into Reality


The graphics will be provided by gtk
as this is a rust project we will be using the gtk-rs crate,
and relying on the system provided gtk libraries.
this will be important to keep aware of if we ever think of porting
to macOS or Windows.

The database will be provided by sqlite
we will be using the sqlite crate to provide a rust native way of talking to sqlite databases

The interactions will be coordinated by systemd via user timers.
we will be using the systemd-user-timers-lib crate to abstract creating and starting systemd user timers.
This is another point to be aware of when we consider porting to macOS or Windows.



### GTK graphics setup

