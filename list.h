#ifndef STRING_UTILS_H
#define STRING_UTILS_H

typedef struct Node{
	int value;
	struct Node *next;
}Node;

typedef struct List{
	Node *head;
	Node *tail;
	unsigned int size;
}List;

List *create_list();
Node *create_node(int value);
void add_first(List *list, int value);
void add_last(List *list, int value);
void remove_first(List *list);
void remove_last(List *list);
void delete_list(List *list);

#endif
