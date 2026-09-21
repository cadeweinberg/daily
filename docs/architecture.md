<!--
 Copyright 2026 cade
 
 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at
 
     https://www.apache.org/licenses/LICENSE-2.0
 
 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.
-->

# Architecture


And for the architecture of the application I am thinking it's essentially a desktop notification on a schedule,
which breings up a little interactable window. This does the work of "asking the questions", and "storing the responses"
in addition, there is an application window which allows the user to configure the notifications, the questions, what responses 
are allowed, etc. 

I am thinking we can use GTK or Qt for the notification and application GUI. plus a SQL database to record the answers, and 
provide the history of responses. I have a hunch that SQLLite will work great for this use case. 

Which is essentially three major components. 

THe database of questions to be asked
The notification which asks and records answers
THe application which queries over the database 

We'll use app to place the source files defining the application interface
The application will need an interface to select which questions are asked,
when the questions are asked, and what kinds of responses to each question are allowed.

The application will further require interfaces to review The responses to the questions
sorted by date. and I would like an interface which allows the graphing of questions which have numeric answers. With axis being the number along Y, and the date along X.

We'll use data to place the source files defining the questions to be asked, 
and with the potential to be asked.
we can store the database in the users XDG cache directory or XDG config directory.
The questions can be stored in the database as strings. 
perhaps we use rows as categories of question.
and we can allow the user to define a new row, by way of a new category.


We'll use the notify directory to define the applet? system notification event
which asks a question and logs the result into the database.

