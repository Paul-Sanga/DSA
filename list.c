#include<stdio.h>
#include<stdlib.h>
#include "list.h"

List *create_list(){
	List *list = malloc(sizeof(List));
	list->head = NULL;
	list->tail = NULL;
	list->size = 0;
	return list;
}

Node *create_node(int value){
	Node *node = malloc(sizeof(Node));
	node->value = value;
	node->next = NULL;
	return node;
}

void add_first(List *list, int value){
	Node *node = create_node(value);
	if(list->size <= 0){
		list->head = node;
		list->tail = list->head;
	}else{
		node->next = list->head;
		list->head = node;
	}
	list->size += 1;
}

void add_last(List *list, int value){
	Node *node = create_node(value);
	if(list->size <= 0){
		list->tail = node;
		list->head = node;
	}else{
		list->tail->next = node;
		list->tail = node;
	}
	list->size ++;
}

void remove_first(List *list){
	if(list->size <= 0){
		printf("Can not remove from an empty list");
	}else if(list->size == 1){
		free(list->head);
		list->head = NULL;
		list->tail = NULL;
	}else{
		Node *node = list->head->next;
		free(list->head);
		list->head = NULL;
		list->head = node;
	}
	list->size --;
}

void remove_last(List *list){
	if(list->size <= 0){
		printf("Can not remove from an empty list");
	}else if(list->size == 1){
		free(list->tail);
		list->tail = NULL;
		list->head = NULL;
	}else{
		Node *node = list->head;
		int i = 0;
		while(node->next->next != NULL){
			node = node->next;
		}
		free(list->tail);
		list->tail = NULL;
		list->tail = node;
	}
	list->size --;
}

void delete_list(List *list){
	if(list->size > 0){
		Node *node = list->head;
		while(node->next != NULL && list->size != 0){
			Node *current_node = node;
			node = current_node->next;
			free(current_node);
			current_node = NULL;
			list->size --;
		}
		free(list);
		list = NULL;
	}else{
		printf("Can not free empty list\n");
	}
}