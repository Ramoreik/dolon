# Prophet
program to find shit on computer and output said shit in a configurable way.

Features:

- configuration:  
	- hints: file that suggests how the program can identify a technology.  
	- scan: file that contains how to get relevant information from that technology.  

- execution:
	- core: tasked with linking the config, modules and main functionality together.  
	- sniffer?: feature that parses hints and validates if a tech is present or not.  
	- locator: feature that is tasked with locating the files necessary to analyze the host.  
	- scanner: feature that uses the configuration provided to find and analyse technologies on the host.  
	- modules: different modules must be available for use, these will most likely be in python, but if need arises we could add more interpreters.  
	- callbacks: scripts called after a scan is performed.  


## Core
Basically main, contains argument parsing, configuration parsing and the binding between all ofther features.

## Sniffer
Feature that focuses strictly on fast and lazy validation of a technology's presence.
It will parse the provide hints file and from this file extract which test they need to perform for each technology to be present.
Once that information is acquired the program can assess the system.

## Locator
Feature that focuses on simply and quickly locating different files on the system.
Ideally this would be able to used with path expansion and other shortcuts, this would allow for the configuration to be very simple.
For example, using file patterns to identify a technology could be represented like:

```
config/config.xml
config//version.xml // this would be expanded to any directory that contains a version.xml file.
```

## Interpreter?
Maybe a feature that pre-processes the yml files could be usefull, especially in a compiled language.
This would make the program crash with a convenient error if a misconfiguration is detected.
stuff such as calling a module that doesn't exist.

## Scanner 
This feature would be tasked with executing the tests to find relevant information for a given technology.
This will be done with the help of the second configuration file, which contains the representation of the tests for technologies in yml.

## Callbacks
This feature would permit the use of callbacks to be executed after a scan was performed. 
This can be usefull in various ways such as sending mails, requests or performing other OS-focused tests for inventory. 
The callbacks should be in a scripting language to allow less experienced users to enjoy the project. (Python)

## Modules
This is the heart of the project, the more modules we have the more this project can do.
Technically this is just like the module engine of Ansible except our focus is not orchestration ? should it be? 


