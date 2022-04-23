# 8. Cross-cutting Concepts

This section describes overall, principal regulations and solution ideas
that are relevant in multiple parts (= cross-cutting) of your system.
Such concepts are often related to multiple building blocks. They can
include many different topics, such as

- domain models

- architecture patterns or design patterns

- rules for using specific technology

- principal, often technical decisions of overall decisions

- implementation rules
# SOLID Principles

## Single Responsibility Principle

### What does it say ?
> Classes should have a single responsibility and thus only a single reason to change.
> The classes you write, should not be a swiss army knife. They should do one thing, and to that one thing well.

### Problem:
`Text` class having 3 attributes - text, author and length
`Text` class also has a `printText()` method to print the text -> **this breaks SRP**

### Solution:
Move out `printText()` method to another class `Printer` (separation of concerns)
`Text` class should not care about printing the text


## Open Closed Principle

### What does it say ?
> Software entities (classes, modules, functions, etc.) should be open for extension, but closed for modification.

### Problem:
`Rectangle` class having 2 attributes - length and width
`AreaCalculator` class having method - `calculateRectangleArea(Rectangle rectangle)`
In future, if we want to calculate area of a Circle then create a `Circle` class with attribute - radius
Add another method in `AreaCalculator` - `calculateCircleArea(Circle circle)`
If new shape needs to be added then `AreaCalculator` has to be modified **again and again!**

### Solution:
Create an interface `Shape` with method - `double calculateArea()`
Now add as many shapes as you want and have the area calculation logic in the classes implementing the Shape interface

```java
Rectangle implements Shape
```

```java
Circle implements Shape
```

```java
Pentagon implements Shape
```

And the `AreaCalculator` class changes now has one method which takes the Shape as an argument :

```java
    public double calculateShapeArea(Shape shape) {
	   return shape.calculateArea();
	}
```


## Liskov Substitution Principle

### What does it say ?
> Sub-types must be behaviorally substitutable for their base types.
> That means objects of the derived class must behave in a manner consistent with the promises made in the base class' contract.

### Problem:
**The Circle-Ellipse Problem or The Square-Rectangle Problem**
A `Square` is ideally a `Rectangle` with equal `width` and `height`
So `Square` can inherit `Rectangle` class having attributes - `width` and `height`
Through `setWidth()` and `setHeight()` methods, `width` and `height` can altered
Calling any of these two methods on an object of type `Square` inside a reference of type `Rectangle` would lead to a `Square` no longer being a `Square`.
Therefore a `Square` is behaviorally not a correct substitution for `Rectangle`!

### Solution:
Correct the inheritance hierarchy by introducing a `Shape` interface

```
	             .------------------.
	             |Shape             |
	             |------------------|
	             |------------------|
	             |+int computeArea()|
	             '------------------'
	                  '         '
	                 '           '
	.---------------'---.  .------'---------------.
	|Square             |  |Rectangle             |
	|-------------------|  |----------------------|
	|-------------------|  |----------------------|
	|+setSides(int side)|  |+setWidth(int width)  |
	|+int computeArea() |  |+setHeight(int height)|
	'-------------------'  |+int computeArea()    |
	                       '----------------------'
```

## Interface Segregation Principle

### What does it say ?
> Many client specific interfaces are better than one general purpose interface.
> Clients should not be forced to depend upon interfaces that they don't use.
> YAGNI - You Ain't Going to Need It.

### Problem:
An old-fashioned printer which can just print will also have to implement unnecessary methods `scan()` and `fax()`
So this interface violates ISP - it is a **"polluted"** or a **"fat"** interface

```java
public interface MachineViolatingISP {
  public void print(Document d);
  public void scan(Document d);
  public void fax(Document d);
}
```

### Solution:
Segregate the general purpose interface into multiple specific (and cohesive) interfaces:
- `Printer : print()`
- `Scanner : scan()`
- `Faxer   : fax()`

```java
Photocopier implements Printer, Scanner
```

```java
Scanner implements Scanner
```

```java
OldFashionedPrinter implements Printer
```

```java
ModernPrinter implements Printer, Scanner, Faxer
```


## Dependency Inversion Principle

### What does it say ?

> High-level modules should not depend on low-level modules.
> Both should depend on abstractions.

> Abstractions should not depend on details.
> Details should depend on abstractions.

### Problem:

```java
LightBulb (class)

void turnOn()
void turnOff()
```

```java
ElectricPowerSwitch (class)

LightBulb lightBulb
boolean on

ElectricPowerSwitch(LightBulb lightBulb)

boolean isOn()
void press()
```

If you see in the code, the `LightBulb` class is hard-coded in `ElectricPowerSwitch`. But, a `switch` should not be tied to a `bulb`.
It should be able to turn on and off other appliances and devices too, say a fan or an AC.


### Solution:

```java
Switch (interface)

boolean isOn()
void press()
```

```java
Switchable (interface)

void turnOn()
void turnOff()
```

```java
ElectricPowerSwitch implements Switch

Switchable device
boolean on
```

```java
ElectricPowerSwitch(Switchable device)

boolean isOn()
void press()
```

```java
LightBulb implements Switchable

@Override turnOn()
@Override turnOff()
```

```java
Fan implements Switchable

@Override turnOn()
@Override turnOff()
```

```java
Client

Switchable bulb = new LightBulb();
Switch bulbSwitch = new ElectricPowerSwitch(bulb);
bulbSwitch.press();
bulbSwitch.press();

Switchable fan = new Fan();
Switch fanSwitch = new ElectricPowerSwitch(fan);
fanSwitch.press();
fanSwitch.press();
```


= CAP Theorem

When we design a distributed system, trading off among CAP (consistency, availability, and partition tolerance) is almost the first thing we want to consider.

* Consistency: all nodes see the same data at the same time
* Availability: a guarantee that every request receives a response about whether it succeeded or failed
* Partition tolerance: system continues to operate despite arbitrary message loss or failure of part of the system

In a distributed context, the choice is between CP and AP.
Unfortunately, CA is just a joke, because single point of failure is a red flag in the real distributed systems world.

To ensure consistency, there are some popular protocols to consider: 2PC, eventual consistency (vector clock + RWN), Paxos, In-Sync Replica, etc.

To ensure availability, we can add replicas for the data. As to components of the whole system, people usually do cold standby, warm standby, hot standby, and active-active to handle the failover.

Source: https://www.puncsky.com/blog/2016/02/14/crack-the-system-design-interview/

== Cassandra
Cassandra is set up to be Available and Partition tolerant.
This means that Cassandra can be distributed on multiple machines (Partition tolerance). +
By having multiple instances available, Cassandra can handle the fact that instances are unavailable and it can spread the load accros the available instances (Availability).




High Availability - Cold, Warm, Hot

Clustering is the most common technique to achieve High availability for any services by introducing redundancy in software, hardware and data.
In a failure the clustering software immediately start the application on the standby system without requiring administrative intervention.
Depending on the type of redundancy in software to be provided for High Availability, clusters can be configured in any of the following configurations:

* Cold Standby: The secondary node acts as backup of another identical primary system. It will be installed and configured only when the primary node breaks down for the first time.
Subsequently, In the case of a failure in the primary the secondary node is powered on and the data restored before finally starting the failed component.
Data from primary system can be backed up on a storage system and restored on secondary system as and when required.
This generally provides a recovery time of a few hours.
* Warm Standby: The software component is installed and available on the secondary node.
The secondary node is up and running. In the case of a failure on the primary node, these software components are started on the secondary node.
This process is usually automated using a cluster manager.Data is regularly mirrored to secondary system using disk based replication or shared disk.
This generally provides a recovery time of a few minutes.
* Hot Standby: Software components are installed and available on both primary and secondary nodes.
The software components on the secondary system are up but will not process data or requests.

Data is mirrored in near real time and both systems will have identical data.
Data replication is typically done through the software’s capabilities.
This generally provides a recovery time of a few seconds.

* Active-Active (Load Balanced): In this method both the primary and secondary systems are active and processing requests in parallel.
Data replication happens through software capabilities and would be bi-directional.
This generally provides a recovery time that is instantaneous.

= Sharding

Sharding is a very important concept which helps the system to keep data into different resources according to the sharding process.

= Scaling
== Horizontal and Vertical Scaling In Databases

The scaling operation can be achieved by adding resources to meet the smaller expectation in the current system, or by adding a new system in the existing one, or both.

image::scaleDB.png[Scale DB]

Source: https://www.geeksforgeeks.org/horizontal-and-vertical-scaling-in-databases/

Vertical Scaling: When new resources are added in the existing system to meet the expectation, it is known as vertical scaling.

Horizontal Scaling: When new server racks are added in the existing system to meet the higher expectation, it is known as horizontal scaling.


