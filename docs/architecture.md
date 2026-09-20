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

WHich is essentially three major components. 

THe database of questions to be asked
The notification which asks and records answers
THe application which queries over the database 
