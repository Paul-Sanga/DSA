#include<stdio.h>
#include "list.h"

int main(int argc, char *argv[]){
	List *list = create_list();
	add_first(list, 10);
	add_first(list, 20);
	add_first(list, 30);
	add_last(list, 40);
	add_last(list, 50);
	remove_first(list);
	remove_last(list);
	delete_list(list);
	printf("The size of list is: %d %d %d", list->size, list->head->value, list->tail->value);
	return 0;
}
