# ODE

## Warning:
This is an experiment I started in college to play around with different database ideas.
As a result it has everything and the kitchen sink.
It is also more of a database framework than a database.
Due to recursive traversal the performance may leave something to be desired.
This is also not really complete.

## About
A database/data processing framework that enables retroactive schemaing and indexing of stored data.
The system also enables automatic event processing.

This will enable querying of event like data sources.
For example querying a camera for frames with cars.

The system can also enable organizing data lakes after creation.
For example if a series of healthcare images is uploaded a plugin can later be written to allow images to be queried and indexed by symptoms shown and date of capture.
Alternately the system could be overlayed onto a system like Amazon S3 by writing an S3 node driver.
This could enable indexing of unindexed systems like S3.

Due to each path having a data type objects can be annotated by writing to the same path with a different type.
These annotations can then be schemaed, indexed, and queried.

Data is stored under a tree structure.
The root of the tree is a network with ODE nodes under it.
In theory queries can be sent to multiple ODE nodes by specifying fuzzy paths.

Currently, versioning is supposed to be explicit.
