![CI](https://github.com/the-gorg/wonky/workflows/CI/badge.svg?branch=main)
``` 
   ▄   ▄                      (._. )
   █ ▄ █ █▀█ █▀▄█ █ █ █ █   
   █▀ ▀█ █▄█ █  █ █▀▄  █   
 Conkys weird terminal cousin
```
 Monitor and display various things by reading stdout from 
 scripts or programs. Have a look at the [example.toml](../main/example.toml) to 
 get started!
 
 ![Screenshot](/media/wonky.png)
 
 Currently has 3 component types, indicator, meter and
 separator.
 
 
 ## Meter basic usage:
 ```toml
    [[widgets]]
    # Type of widget
    type            = "Meter"
    title           = "I rate"
    
    # Unit of messurement
    unit            = " m8" 
    
    # Themes for now:
    # 0 ▀▀▀▀▀▀▀▀▀▀ 
    # 1 [====----]
    theme           = 0
    
    # Text to the left of the bar
    # prefix          = "something"

    # Display reading and title above the bar
    # either of these will cause the meter to
    # take up two vertical spaces.
    reading         = true
    # Hide the meter
    meter           = true

    max_command     = ["echo", "8"] 
    value_command   = ["echo", "8"] 
    
    # How often component should be updated in seconds
    frequency       = 60
    # Horizontal alignment
    right           = false
    # Vertical alignment
    bottom          = false
 ```
 
 ## Bash script:
 ```toml
    [[widgets]]
    type            = "Meter"
    title           = ""
    unit            = "c" 
    theme           = 1
    
    prefix          = "cputemp"

    reading         = true
    meter           = true

    max_command     = ["bash", "~/scripts/cputemp", "max"] 
    value_command   = ["bash", "~/scripts/cputemp"] 
    
    frequency       = 60
    right           = false
    bottom          = false
 ```
 
