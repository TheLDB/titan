
#  Titan

  

#  **What is this**

This is a discord bot to monitor OpenSea collections and get info about them, including:

- Floor Price

- Activity

- Sales per Hour

- And more...

  

#  **Usage**

Per Discord Bot guidelines, this bot uses *slash* commands.

  

What does this mean?

  

Instead of using standard ``!floor squishy``, you would use ``/floor squishy``

  

*will add images when i make it*

  

#  Commands

- Get Collection Info
	- ``/collection *collection-slug*``
	- How do I find a collection slug?
		- https://opensea.io/collection/**squishy-squad**
		- The bolded part is the slug, so the short name after /collection/
		
- Monitor Collection
	- ``/new-watcher *collection-slug* *time-interval*``
	- Will send a message to the channel it was set, based on how fast or slow the time-interval is. Will also send a message when the floor price or activity jumps by more than 10%.
		- This will be customizable in future versions
	
- Get Trending Collections
	- ``/trending``
	- Will send a message displaying the top 10 trending collections, based on activity
	
- Get price and best bid on an item
	- ``/item *link*``
	- Will send a message displaying the item, its current buy now price (if applicable), and the highest big (if applicable)

### More Coming Soon
